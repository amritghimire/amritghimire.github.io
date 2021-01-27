import api from "@/utils/api";
import { BLOG_LIST } from "@/constants/routes";
import { Blog, BlogData } from "@/models/blog";

export const listBlog = async (filters: object): Promise<Blog[]> => {
  const params = {
    status: "published",
    limit: 10,
    offset: 0,
    ...filters
  };
  const response = await api.get(BLOG_LIST, {
    params
  });
  if (response.status >= 400) {
    return [];
  }
  const results = response.data["results"] || [];
  return results.map((blog: object) => new Blog(blog as BlogData));
};

export const getBlog = async (slug: string): Promise<Blog | null> => {
  try {
    const response = await api.get(`${BLOG_LIST}${slug}/`);
    if (response.status >= 400) {
      return null;
    }
    const results = response.data || {};
    return new Blog(results as BlogData);
  } catch (e) {
    return null;
  }
};
