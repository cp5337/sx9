import { useState, useRef, useCallback, useEffect } from 'react';

interface VoiceAgent {
  id: string;
  name: string;
  personality: string;
  voiceId: string; // ElevenLabs voice ID
  symbol: string;
  color: string;
}

interface ConversationMessage {
  id: string;
  agentId: string;
  content: string;
  timestamp: Date;
  type: 'user_voice' | 'agent_voice' | 'user_text' | 'agent_text';
  audioUrl?: string;
}

export const useVoiceConversation = () => {
  const [isListening, setIsListening] = useState(false);
  const [isSpeaking, setIsSpeaking] = useState(false);
  const [activeAgent, setActiveAgent] = useState<VoiceAgent | null>(null);
  const [conversation, setConversation] = useState<ConversationMessage[]>([]);
  const [isProcessing, setIsProcessing] = useState(false);

  const mediaRecorderRef = useRef<MediaRecorder | null>(null);
  const audioContextRef = useRef<AudioContext | null>(null);
  const currentAudioRef = useRef<HTMLAudioElement | null>(null);

  // Agent personalities with distinct voices
  const voiceAgents: VoiceAgent[] = [
    {
      id: 'claude',
      name: 'Claude',
      personality: 'analytical, precise, helpful',
      voiceId: 'EXAVITQu4vr4xnSDxMaL', // Mature male, professional
      symbol: '⚡',
      color: '#3B82F6'
    },
    {
      id: 'copilot',
      name: 'Copilot',
      personality: 'energetic, code-focused, collaborative',
      voiceId: '21m00Tcm4TlvDq8ikWAM', // Young female, enthusiastic
      symbol: '🚀',
      color: '#8B5CF6'
    },
    {
      id: 'cursor',
      name: 'Cursor',
      personality: 'direct, efficient, goal-oriented',
      voiceId: 'AZnzlk1XvdvUeBnXmlld', // Deep male, authoritative
      symbol: '🎯',
      color: '#10B981'
    },
    {
      id: 'codeium',
      name: 'Codeium',
      personality: 'creative, innovative, experimental',
      voiceId: 'ErXwobaYiN019PkySvjV', // Soft female, creative
      symbol: '💎',
      color: '#06B6D4'
    }
  ];

  // Initialize audio context
  useEffect(() => {
    audioContextRef.current = new AudioContext();
    return () => {
      audioContextRef.current?.close();
    };
  }, []);

  // Start voice recording
  const startListening = useCallback(async (targetAgent?: VoiceAgent) => {
    try {
      const stream = await navigator.mediaDevices.getUserMedia({
        audio: {
          echoCancellation: true,
          noiseSuppression: true,
          autoGainControl: true,
          sampleRate: 16000
        }
      });

      setActiveAgent(targetAgent || voiceAgents[0]);
      setIsListening(true);

      const mediaRecorder = new MediaRecorder(stream, {
        mimeType: 'audio/webm;codecs=opus'
      });

      const audioChunks: BlobPart[] = [];

      mediaRecorder.ondataavailable = (event) => {
        if (event.data.size > 0) {
          audioChunks.push(event.data);
        }
      };

      mediaRecorder.onstop = async () => {
        const audioBlob = new Blob(audioChunks, { type: 'audio/webm' });
        await processVoiceInput(audioBlob, targetAgent || voiceAgents[0]);
        stream.getTracks().forEach(track => track.stop());
      };

      mediaRecorderRef.current = mediaRecorder;
      mediaRecorder.start(1000); // Collect data every second

    } catch (error) {
      console.error('Error starting voice recording:', error);
      setIsListening(false);
    }
  }, []);

  // Stop voice recording
  const stopListening = useCallback(() => {
    if (mediaRecorderRef.current && isListening) {
      mediaRecorderRef.current.stop();
      setIsListening(false);
    }
  }, [isListening]);

  // Process voice input through the full pipeline
  const processVoiceInput = useCallback(async (audioBlob: Blob, agent: VoiceAgent) => {
    setIsProcessing(true);

    try {
      // Step 1: Speech-to-Text using Whisper.cpp
      const transcriptionText = await transcribeAudio(audioBlob);

      // Add user message to conversation
      const userMessage: ConversationMessage = {
        id: `msg-${Date.now()}`,
        agentId: 'user',
        content: transcriptionText,
        timestamp: new Date(),
        type: 'user_voice'
      };
      setConversation(prev => [...prev, userMessage]);

      // Step 2: Process with agent AI (context-aware)
      const agentResponse = await getAgentResponse(transcriptionText, agent, conversation);

      // Step 3: Convert response to speech
      const audioUrl = await synthesizeSpeech(agentResponse, agent.voiceId);

      // Add agent message to conversation
      const agentMessage: ConversationMessage = {
        id: `msg-${Date.now()}-agent`,
        agentId: agent.id,
        content: agentResponse,
        timestamp: new Date(),
        type: 'agent_voice',
        audioUrl
      };
      setConversation(prev => [...prev, agentMessage]);

      // Step 4: Play agent response
      await playAgentResponse(audioUrl, agent);

    } catch (error) {
      console.error('Error processing voice input:', error);
    } finally {
      setIsProcessing(false);
    }
  }, [conversation]);

  // Transcribe audio using Whisper.cpp (local)
  const transcribeAudio = async (audioBlob: Blob): Promise<string> => {
    const formData = new FormData();
    formData.append('audio', audioBlob, 'recording.webm');

    // Use local Whisper.cpp server
    const response = await fetch('http://localhost:8080/transcribe', {
      method: 'POST',
      body: formData
    });

    if (!response.ok) {
      throw new Error('Transcription failed');
    }

    const result = await response.json();
    return result.text || '';
  };

  // Get contextual response from agent
  const getAgentResponse = async (
    userInput: string,
    agent: VoiceAgent,
    conversationHistory: ConversationMessage[]
  ): Promise<string> => {

    // Build context from conversation history
    const context = conversationHistory
      .slice(-6) // Last 6 messages for context
      .map(msg => `${msg.agentId}: ${msg.content}`)
      .join('\n');

    const systemPrompt = `You are ${agent.name}, a ${agent.personality} AI assistant in the Synaptix9 Command Center.

Context from recent conversation:
${context}

Respond in character as ${agent.name}. Keep responses conversational, under 50 words, and actionable.
Current screen context: You have access to all SX9 systems - crates, metrics, DevOps, etc.

User said: "${userInput}"

Respond as ${agent.name}:`;

    // Call your preferred AI API (Claude, OpenAI, etc.)
    const response = await fetch('/api/ai/chat', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        messages: [{ role: 'system', content: systemPrompt }],
        model: 'claude-3-sonnet',
        max_tokens: 150
      })
    });

    const result = await response.json();
    return result.content || "I'm processing that request...";
  };

  // Convert text to speech using ElevenLabs
  const synthesizeSpeech = async (text: string, voiceId: string): Promise<string> => {
    const response = await fetch(`https://api.elevenlabs.io/v1/text-to-speech/${voiceId}`, {
      method: 'POST',
      headers: {
        'Accept': 'audio/mpeg',
        'Content-Type': 'application/json',
        'xi-api-key': process.env.REACT_APP_ELEVENLABS_API_KEY || ''
      },
      body: JSON.stringify({
        text,
        model_id: 'eleven_monolingual_v1',
        voice_settings: {
          stability: 0.5,
          similarity_boost: 0.75,
          style: 0.5,
          use_speaker_boost: true
        }
      })
    });

    if (!response.ok) {
      throw new Error('Speech synthesis failed');
    }

    const audioBlob = await response.blob();
    return URL.createObjectURL(audioBlob);
  };

  // Play agent response with visual feedback
  const playAgentResponse = async (audioUrl: string, agent: VoiceAgent): Promise<void> => {
    return new Promise((resolve, reject) => {
      setIsSpeaking(true);

      const audio = new Audio(audioUrl);
      currentAudioRef.current = audio;

      audio.onended = () => {
        setIsSpeaking(false);
        setActiveAgent(null);
        resolve();
      };

      audio.onerror = () => {
        setIsSpeaking(false);
        setActiveAgent(null);
        reject(new Error('Audio playback failed'));
      };

      audio.play().catch(reject);
    });
  };

  // Stop current agent speech
  const stopSpeaking = useCallback(() => {
    if (currentAudioRef.current) {
      currentAudioRef.current.pause();
      currentAudioRef.current.currentTime = 0;
      setIsSpeaking(false);
      setActiveAgent(null);
    }
  }, []);

  // Quick agent activation
  const talkToAgent = useCallback(async (agentId: string) => {
    const agent = voiceAgents.find(a => a.id === agentId);
    if (agent) {
      await startListening(agent);
    }
  }, [startListening]);

  // Voice command shortcuts
  const handleVoiceCommand = useCallback(async (command: string) => {
    const lowerCommand = command.toLowerCase();

    // Route to specific agents based on command context
    if (lowerCommand.includes('code') || lowerCommand.includes('debug')) {
      await talkToAgent('copilot');
    } else if (lowerCommand.includes('deploy') || lowerCommand.includes('devops')) {
      await talkToAgent('cursor');
    } else if (lowerCommand.includes('creative') || lowerCommand.includes('design')) {
      await talkToAgent('codeium');
    } else {
      await talkToAgent('claude'); // Default to Claude
    }
  }, [talkToAgent]);

  return {
    // State
    isListening,
    isSpeaking,
    isProcessing,
    activeAgent,
    conversation,
    voiceAgents,

    // Actions
    startListening,
    stopListening,
    stopSpeaking,
    talkToAgent,
    handleVoiceCommand,

    // Conversation management
    clearConversation: () => setConversation([]),
    getConversationSummary: () => conversation.slice(-10)
  };
};