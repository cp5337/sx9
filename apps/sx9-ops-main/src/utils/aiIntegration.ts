import axios from 'axios';
import { config } from '@/config/config';

const OPENAI_API_URL = 'https://api.openai.com/v1/chat/completions';
const WOLFRAM_API_URL = 'https://api.wolframalpha.com/v2/query';

export const queryChatGPT = async (prompt: string): Promise<string> => {
  try {
    const response = await axios.post(OPENAI_API_URL, {
      model: "gpt-3.5-turbo",
      messages: [{ role: "user", content: prompt }]
    }, {
      headers: {
        'Authorization': `Bearer ${config.openAiApiKey}`,
        'Content-Type': 'application/json'
      }
    });
    return response.data.choices[0].message.content;
  } catch (error) {
    console.error('Error querying ChatGPT:', error);
    return 'Error: Unable to get response from ChatGPT';
  }
};

export const queryWolframAlpha = async (query: string): Promise<string> => {
  try {
    const response = await axios.get(WOLFRAM_API_URL, {
      params: {
        input: query,
        appid: config.wolframAppId,
        output: 'json'
      }
    });
    // Parse the Wolfram Alpha response and extract relevant information
    // This is a simplified example and may need to be adjusted based on the actual response structure
    return response.data.queryresult.pods[1].subpods[0].plaintext;
  } catch (error) {
    console.error('Error querying Wolfram Alpha:', error);
    return 'Error: Unable to get response from Wolfram Alpha';
  }
};

// Note: For Gemini, as of my last update, there wasn't a public API available.
// When it becomes available, you can implement a similar function here.

export const queryGemini = async (prompt: string): Promise<string> => {
  // This is a placeholder function. Replace with actual implementation when Gemini API is available.
  console.log('Querying Gemini:', prompt);
  return 'Gemini response placeholder';
};