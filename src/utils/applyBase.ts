import { config } from "@/config";

export default function(path: string) {
  if (path && path.startsWith("http")) {
    return path;
  } else {
    return config.baseUrl + path;
  }
}
