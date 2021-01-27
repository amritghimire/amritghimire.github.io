import axios from "axios";
import { config, siteConfigurations, metaConfigurations } from "@/config";

export function getWebHost() {
  const host: string = window.location.hostname;
  return siteConfigurations[host] || "default";
}

export function getMeta(): { title: string; [index: string]: string } {
  const host: string = window.location.hostname;
  return metaConfigurations[host] || { title: config.websiteTitle };
}

const instance = axios.create({
  baseURL: `${config.baseUrl}/api`,
  headers: { "Origin-Website": getWebHost() }
});

export default instance;
