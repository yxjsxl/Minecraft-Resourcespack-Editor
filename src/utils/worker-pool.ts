interface Task {
  id: string;
  type: string;
  data: any;
  resolve: (result: any) => void;
  reject: (error: Error) => void;
}

class WorkerPool {
  private workers: Worker[] = [];
  private availableWorkers: Worker[] = [];
  private taskQueue: Task[] = [];
  private pendingTasks: Map<string, Task> = new Map();
  private workerCount: number;

  constructor(workerCount?: number) {
    const cpuCount = navigator.hardwareConcurrency || 4;
    this.workerCount = workerCount || Math.min(Math.max(cpuCount, 2), 16);
    
    this.initializeWorkers();
    
    console.log(`[Worker Pool] 初始化了 ${this.workerCount} 个Worker`);
  }

  private initializeWorkers(): void {
    for (let i = 0; i < this.workerCount; i++) {
      try {
        const worker = new Worker(
          new URL('../workers/image-worker.ts', import.meta.url),
          { type: 'module' }
        );
        
        worker.onmessage = (event) => this.handleWorkerMessage(worker, event);
        worker.onerror = (error) => this.handleWorkerError(worker, error);
        
        this.workers.push(worker);
        this.availableWorkers.push(worker);
      } catch (error) {
        console.error(`[Worker Pool] 创建Worker ${i} 失败:`, error);
      }
    }
  }

  private handleWorkerMessage(worker: Worker, event: MessageEvent): void {
    const { id, type, result, error } = event.data;
    
    const task = this.pendingTasks.get(id);
    if (!task) {
      console.warn(`[Worker Pool] 收到未知任务的响应: ${id}`);
      return;
    }
    
    this.pendingTasks.delete(id);
    this.availableWorkers.push(worker);
    
    if (error) {
      task.reject(new Error(error));
    } else {
      task.resolve(result);
    }
    
    this.processNextTask();
  }

  private handleWorkerError(worker: Worker, error: ErrorEvent): void {
    console.error('[Worker Pool] Worker错误:', error);
    
    this.pendingTasks.forEach((task, id) => {
      task.reject(new Error('Worker错误'));
      this.pendingTasks.delete(id);
    });
    
    const index = this.workers.indexOf(worker);
    if (index > -1) {
      worker.terminate();
      
      try {
        const newWorker = new Worker(
          new URL('../workers/image-worker.ts', import.meta.url),
          { type: 'module' }
        );
        
        newWorker.onmessage = (event) => this.handleWorkerMessage(newWorker, event);
        newWorker.onerror = (error) => this.handleWorkerError(newWorker, error);
        
        this.workers[index] = newWorker;
        this.availableWorkers.push(newWorker);
      } catch (err) {
        console.error('[Worker Pool] 重新创建Worker失败:', err);
      }
    }
  }

  private processNextTask(): void {
    if (this.taskQueue.length === 0 || this.availableWorkers.length === 0) {
      return;
    }
    
    const task = this.taskQueue.shift()!;
    const worker = this.availableWorkers.shift()!;
    
    this.pendingTasks.set(task.id, task);
    
    worker.postMessage({
      id: task.id,
      type: task.type,
      data: task.data,
    });
  }

  execute<T = any>(type: string, data: any): Promise<T> {
    return new Promise((resolve, reject) => {
      const task: Task = {
        id: `task_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
        type,
        data,
        resolve,
        reject,
      };
      
      this.taskQueue.push(task);
      this.processNextTask();
    });
  }

  async executeBatch<T = any>(tasks: Array<{ type: string; data: any }>): Promise<T[]> {
    const promises = tasks.map(task => this.execute<T>(task.type, task.data));
    return Promise.all(promises);
  }

  getStats() {
    return {
      totalWorkers: this.workerCount,
      availableWorkers: this.availableWorkers.length,
      pendingTasks: this.pendingTasks.size,
      queuedTasks: this.taskQueue.length,
    };
  }

  terminate(): void {
    this.workers.forEach(worker => worker.terminate());
    this.workers = [];
    this.availableWorkers = [];
    this.taskQueue = [];
    this.pendingTasks.clear();
  }
}

export const workerPool = new WorkerPool();

export const processImageInWorker = (imageData: ImageData) => {
  return workerPool.execute('process-image', { imageData });
};

export const calculateHistogramInWorker = (imageData: ImageData) => {
  return workerPool.execute('calculate-histogram', { imageData });
};

export const applyFilterInWorker = (
  imageData: ImageData,
  filterType: string,
  params: any
) => {
  return workerPool.execute('apply-filter', { imageData, filterType, params });
};

export const resizeInWorker = (
  imageData: ImageData,
  width: number,
  height: number
) => {
  return workerPool.execute('resize', { imageData, width, height });
};