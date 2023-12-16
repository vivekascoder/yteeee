import axios, { Axios, AxiosInstance } from "axios";
import { env } from "process";
import dotenv, { config } from "dotenv";

dotenv.config();

interface IApi {}
type SummarizeVideoResponseType = {};

class Api implements IApi {
  private axios: AxiosInstance;
  constructor() {
    this.axios = axios.create({
      baseURL: env.SITE_URL,
    });
  }

  async summarizeVideo(ytUtl: string): Promise<SummarizeVideoResponseType> {
    const summary = await this.axios.post(
      "/summarize_video",
      {
        video_url: ytUtl,
      },
      {
        headers: {
          "Content-Type": "application/json",
        },
      }
    );
  }
}
