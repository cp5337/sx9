import { apiService } from '@/services/api/ApiService';
import type { ThreatActor, CTASTask } from '@/types/tasks';

// Database utility functions
export const getThreatActors = async (): Promise<ThreatActor[]> => {
  try {
    const response = await apiService.get<ThreatActor[]>('/threat-actors');
    return response.data || [];
  } catch (error) {
    console.error('Error fetching threat actors:', error);
    return [];
  }
};

export const createThreatActor = async (name: string, type: string): Promise<ThreatActor | null> => {
  try {
    const response = await apiService.post<ThreatActor>('/threat-actors', { name, type });
    return response.data || null;
  } catch (error) {
    console.error('Error creating threat actor:', error);
    return null;
  }
};

export const getCTASTasks = async (): Promise<CTASTask[]> => {
  try {
    const response = await apiService.get<CTASTask[]>('/tasks');
    return response.data || [];
  } catch (error) {
    console.error('Error fetching CTAS tasks:', error);
    return [];
  }
};

export const createCTASTask = async (
  number: string,
  title: string,
  description: string,
  relatedActorId?: string
): Promise<CTASTask | null> => {
  try {
    const response = await apiService.post<CTASTask>('/tasks', {
      number,
      title,
      description,
      status: 'Pending',
      relatedActorId,
      isSection: false
    });
    return response.data || null;
  } catch (error) {
    console.error('Error creating CTAS task:', error);
    return null;
  }
};

export const updateTaskStatus = async (
  taskId: string,
  newStatus: 'Pending' | 'In Progress' | 'Completed'
): Promise<CTASTask | null> => {
  try {
    const response = await apiService.patch<CTASTask>(`/tasks/${taskId}`, { status: newStatus });
    return response.data || null;
  } catch (error) {
    console.error('Error updating task status:', error);
    return null;
  }
};

export const getTasksWithRelatedActors = async (): Promise<(CTASTask & { actorName?: string })[]> => {
  try {
    const response = await apiService.get<(CTASTask & { actorName?: string })[]>('/tasks/with-actors');
    return response.data || [];
  } catch (error) {
    console.error('Error fetching tasks with actors:', error);
    return [];
  }
};