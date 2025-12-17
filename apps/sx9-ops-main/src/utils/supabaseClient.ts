// Supabase client for local PostgREST or cloud Supabase
import { createClient } from "@supabase/supabase-js";
import { CTAS7_API_ENDPOINTS } from "@/services/ctas7-api-integration";

const supabaseUrl = import.meta.env.VITE_SUPABASE_URL || CTAS7_API_ENDPOINTS.supabase;
const supabaseAnonKey =
  import.meta.env.VITE_SUPABASE_ANON_KEY ||
  "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZS1kZW1vIiwicm9sZSI6ImFub24iLCJleHAiOjE5ODM4MTI5OTZ9.CRXP1A7WOeoJeXxjNni43kdQwgnWNReilDMblYTn_I0";

// Check if we are running against local PostgREST (usually port 18300 or 3000)
const isLocalPostgREST = supabaseUrl.includes("localhost") || supabaseUrl.includes("127.0.0.1");

export const isDemoMode = !supabaseUrl || supabaseUrl === "your_supabase_url_here";

// Create Supabase client
// For local PostgREST, we use a dummy anon key (PostgREST doesn't require auth)
// For cloud Supabase, use the real anon key from env
export const supabase = createClient(supabaseUrl, supabaseAnonKey, {
  auth: {
    persistSession: false, // Disable auth for local PostgREST
    autoRefreshToken: false,
    detectSessionInUrl: false,
  },
  db: {
    schema: "public",
  },
  global: {
    headers: {
      apikey: supabaseAnonKey, // PostgREST expects apikey header
      Authorization: `Bearer ${supabaseAnonKey}`, // Some PostgREST configs need this
    },
  },
  // Disable JWT verification for local PostgREST
  realtime: {
    params: {
      eventsPerSecond: 10,
    },
  },
});

// Test connection on init
if (!isDemoMode) {
  supabase
    .from("tasks")
    .select("count")
    .limit(1)
    .then(
      ({ error }) => {
        if (error && !error.message.includes("does not exist")) {
          console.warn("⚠️ Supabase connection warning:", error.message);
          console.log("💡 Using local PostgREST at", supabaseUrl);
        } else {
          console.log("✅ Supabase connected:", supabaseUrl);
        }
      },
      err => {
        console.warn("⚠️ Supabase connection failed, using demo mode", err);
      }
    );
}
