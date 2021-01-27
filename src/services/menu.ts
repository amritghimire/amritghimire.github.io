import api from "@/utils/api";
import { MENU_LIST } from "@/constants/routes";
import { Menu, MenuData } from "@/models/menu";

export const listMenu = async (): Promise<Menu[]> => {
  const response = await api.get(MENU_LIST);
  if (response.status >= 400) {
    return [];
  }
  const results = response.data["results"] || [];
  return results.map((menu: object) => new Menu(menu as MenuData));
};
