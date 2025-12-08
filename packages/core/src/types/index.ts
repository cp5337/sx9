export interface Persona {
  id: string;
  name: string;
  role: string;
  model: string;
  avatar: string;
  status: 'online' | 'busy' | 'away' | 'offline';
  lastSeen: string;
  capabilities: string[];
}

export interface Message {
  id: string;
  senderId: string;
  recipientId?: string;
  channelId?: string;
  content: string;
  timestamp: string;
  type: 'text' | 'voice' | 'system' | 'file';
  attachments?: Attachment[];
  voiceData?: VoiceMessage;
}

export interface VoiceMessage {
  duration: number;
  waveform: number[];
  isPlaying: boolean;
}

export interface Attachment {
  id: string;
  name: string;
  type: string;
  size: number;
  url: string;
}

export interface Channel {
  id: string;
  name: string;
  type: 'direct' | 'group' | 'system';
  participants: string[];
  lastMessage?: Message;
  unreadCount: number;
}

export interface Task {
  id: string;
  title: string;
  description: string;
  assignedTo: string;
  priority: 'critical' | 'high' | 'medium' | 'low';
  status: 'todo' | 'in-progress' | 'review' | 'done';
  dueDate: string;
  tags: string[];
  createdAt?: string;
  updatedAt?: string;
}

export interface SystemMetric {
  id: string;
  name: string;
  value: number;
  unit: string;
  trend: 'up' | 'down' | 'stable';
  status: 'healthy' | 'warning' | 'critical';
}

export interface ApiEndpoint {
  method: 'GET' | 'POST' | 'PUT' | 'DELETE';
  path: string;
  description: string;
}