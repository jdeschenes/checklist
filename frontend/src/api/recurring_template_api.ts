import {
  CreateRecurringTemplateRequest,
  UpdateRecurringTemplateRequest,
  RecurringTemplateResponse,
  ListRecurringTemplatesResponse,
} from ".";
import {
  authenticatedPost,
  authenticatedGet,
  authenticatedPut,
  authenticatedDelete,
} from "./authenticated-client";

export const BackendRecurringTemplateAPI = {
  CreateRecurringTemplate: async (
    todo_name: string,
    r: CreateRecurringTemplateRequest
  ): Promise<RecurringTemplateResponse> => {
    return await authenticatedPost<RecurringTemplateResponse>(
      `/todo/${todo_name}/recurring`,
      r
    );
  },

  ListRecurringTemplates: async (
    todo_name: string
  ): Promise<ListRecurringTemplatesResponse> => {
    return await authenticatedGet<ListRecurringTemplatesResponse>(
      `/todo/${todo_name}/recurring`
    );
  },

  GetRecurringTemplate: async (
    todo_name: string,
    template_id: string
  ): Promise<RecurringTemplateResponse> => {
    return await authenticatedGet<RecurringTemplateResponse>(
      `/todo/${todo_name}/recurring/${template_id}`
    );
  },

  UpdateRecurringTemplate: async (
    todo_name: string,
    template_id: string,
    r: UpdateRecurringTemplateRequest
  ): Promise<RecurringTemplateResponse> => {
    return await authenticatedPut<RecurringTemplateResponse>(
      `/todo/${todo_name}/recurring/${template_id}`,
      r
    );
  },

  DeleteRecurringTemplate: async (
    todo_name: string,
    template_id: string
  ): Promise<void> => {
    await authenticatedDelete(`/todo/${todo_name}/recurring/${template_id}`);
  },
};
