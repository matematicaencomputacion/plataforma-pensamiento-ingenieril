export const API_BASE_URL = "http://localhost:8080";
export const DEMO_STUDENT_ID = "demo-user";

export type TrackType = "micro_paso" | "reto_ingenieril";

export type Level = {
  id: number;
  title: string;
  statement: string;
  track_type: TrackType;
  evaluation_prompt?: string;
};

export type EvaluateResponse = {
  passed: boolean;
  feedback?: string;
};

export function trackLabel(trackType: TrackType): string {
  switch (trackType) {
    case "micro_paso":
      return "Micro-paso";
    case "reto_ingenieril":
      return "Reto Ingenieril";
    default: {
      const _exhaustive: never = trackType;
      return _exhaustive;
    }
  }
}
