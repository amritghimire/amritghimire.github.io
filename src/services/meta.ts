import api from "@/utils/api";
import { META_LIST } from "@/constants/routes";
import { MetaValue } from "@/models/meta";

export const listMeta = async (): Promise<MetaValue[]> => {
  const params = {
    limit: 100
  };
  const response = await api.get(META_LIST, {
    params
  });
  if (response.status >= 400) {
    return [];
  }
  const results = response.data["results"] || [];
  return results as Array<MetaValue>;
};
