import api from "@/utils/api";
import { CONTACT_FORM } from "@/constants/routes";
import { Contents } from "@/models/contents";

export const sendContact = async (
  contact: Contents,
  entry: object
): Promise<boolean> => {
  if (!contact.id) {
    return false;
  }
  const url = CONTACT_FORM + contact.id + "/";
  try {
    const response = await api.post(url, entry);
    return response.status < 400;
  } catch (e) {
    return false;
  }
};
