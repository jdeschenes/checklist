import { BASE_URL } from ".";

export interface AuthenticatedFetchOptions extends RequestInit {
  skipAuth?: boolean;
}

let authToken: string | null = null;
let onAuthError: (() => void) | null = null;

export const setAuthToken = (token: string | null) => {
  authToken = token;
};

export const setAuthErrorHandler = (handler: () => void) => {
  onAuthError = handler;
};

export const getAuthToken = () => authToken;

export const authenticatedFetch = async (
  url: string,
  options: AuthenticatedFetchOptions = {}
): Promise<Response> => {
  const { skipAuth = false, ...fetchOptions } = options;
  let token: string | null = null;
  if (!skipAuth) {
    token = authToken;
    if (!token && typeof window !== "undefined") {
      token = localStorage.getItem("checklist_auth_token");
    }
    if (token) {
      authToken = token;
    }
  }

  // Add base URL if not already present
  const fullUrl = url.startsWith("http") ? url : `${BASE_URL}${url}`;

  // Add authentication header if we have a token and not skipping auth
  if (!skipAuth && authToken) {
    fetchOptions.headers = {
      ...fetchOptions.headers,
      Authorization: `Bearer ${authToken}`,
    };
  }

  const response = await fetch(fullUrl, fetchOptions);

  // Check for authentication errors
  if ((response.status === 401 || response.status === 403) && token) {
    // Clear stored auth data
    authToken = null;
    localStorage.removeItem("checklist_auth_token");
    localStorage.removeItem("checklist_auth_user");

    // Trigger auth error handler if available
    if (onAuthError) {
      onAuthError();
    }

    throw new Error(`Authentication failed: ${response.status}`);
  }

  return response;
};

export const authenticatedFetchJSON = async <T = unknown>(
  url: string,
  options: AuthenticatedFetchOptions = {}
): Promise<T> => {
  const response = await authenticatedFetch(url, options);

  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }

  return response.json() as Promise<T>;
};

export const authenticatedPost = async <T = unknown>(
  url: string,
  data?: unknown,
  options: AuthenticatedFetchOptions = {}
): Promise<T> => {
  return authenticatedFetchJSON<T>(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      ...options.headers,
    },
    body: data ? JSON.stringify(data) : undefined,
    ...options,
  });
};

export const authenticatedGet = async <T = unknown>(
  url: string,
  options: AuthenticatedFetchOptions = {}
): Promise<T> => {
  return authenticatedFetchJSON<T>(url, {
    method: "GET",
    ...options,
  });
};

export const authenticatedPut = async <T = unknown>(
  url: string,
  data?: unknown,
  options: AuthenticatedFetchOptions = {}
): Promise<T> => {
  return authenticatedFetchJSON<T>(url, {
    method: "PUT",
    headers: {
      "Content-Type": "application/json",
      ...options.headers,
    },
    body: data ? JSON.stringify(data) : undefined,
    ...options,
  });
};

export const authenticatedDelete = async (
  url: string,
  options: AuthenticatedFetchOptions = {}
): Promise<void> => {
  const response = await authenticatedFetch(url, {
    method: "DELETE",
    ...options,
  });

  if (!response.ok) {
    throw new Error(`HTTP error! status: ${response.status}`);
  }
};
