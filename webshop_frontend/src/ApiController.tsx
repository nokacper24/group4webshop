import {
  Company,
  Description,
  License,
  FullLicenseInfo,
  MeUser,
  PartialUser,
  Product,
  Testimonial,
  User,
} from "./Interfaces";

let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT;
// Check if we are in production mode
if (import.meta.env.PROD) {
  baseUrl = "";
}

/**
 * Error for failed fetches.
 * Throw it if response is not as expected.  
 * This error contains the status code as well as the status as text.
 * Status code can be used to determine what went wrong, and act accordingly.
 * @extends Error
 */
export class FetchError extends Error {
  status: number;
  statusText: string;
  /**
   * Create a new product error.
   * @param message The error message.
   * @param status The HTTP status code.
   * @param statusText The HTTP status text.
   */
  constructor(message: string, status: number, statusText: string) {
    super(message);
    this.name = "FetchError";
    this.status = status;
    this.statusText = statusText;
  }
}

/**
 * Get a specific user.
 *
 * @param userId The ID of the user.
 * @returns The user.
 */
export const fetchUser = async (userId: string) => {
  const response = await fetch(`${baseUrl}/api/priv/users/${userId}`);
  if (response.ok) {
    const data: User = await response.json();
    return data;
  } else {
    throw new Error("Could not fetch user.");
  }
};

/**
 * Get the current/logged in user.
 *
 * @returns The current user.
 */
export const fetchMe = async () => {
  const response = await fetch(`${baseUrl}/api/priv/me`, {
    method: "GET",
    credentials: "include",
  });
  if (response.ok) {
    const data: MeUser = await response.json();
    return data;
  } else {
    throw new FetchError(
      "Could not fetch current user.",
      response.status,
      response.statusText
    );
  }
};

/**
 * Update the user information for a specific user.
 *
 * @param userId The ID of the user.
 * @param email The new email.
 * @returns The response from the fetch request.
 */
export const patchPartialUser = async (userId: string, email?: string) => {
  let user: PartialUser = {};
  if (email) {
    user.email = email;
  }

  return await fetch(`${baseUrl}/api/priv/users/${userId}`, {
    method: "PATCH",
    headers: {
      Accept: "application:json",
      "Content-Type": "application/json",
    },
    body: JSON.stringify(user),
  });
};

/**
 * Reset a user's password.
 *
 * @param email The user's e-mail address.
 */
export const resetPassword = async (email: string) => {
  return await fetch(`${baseUrl}/api/reset_password`, {
    method: "POST",
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      email: email,
    }),
  });
};

/**
 * Get all users for a specific company.
 *
 * @param companyId The ID of the company.
 * @returns The users from the company.
 */
export const fetchCompanyUsers = async (companyId: string) => {
  const response = await fetch(
    `${baseUrl}/api/priv/companies/${companyId}/users`
  );
  if (response.ok) {
    const data: User[] = await response.json();
    return data;
  } else {
    throw new Error("Could not fetch users.");
  }
};

/**
 * Get a specific license.
 *
 * @param licenseId The ID of the license.
 * @returns The license.
 */
export const fetchLicense = async (licenseId: string) => {
  const response = await fetch(`${baseUrl}/api/priv/licenses/${licenseId}`);
  if (response.ok) {
    const data: License = await response.json();
    return data;
  } else {
    throw new Error("Could not fetch license.");
  }
};

/**
 * Get all licenses' information.
 *
 * @returns All licenses' information.
 */
export const fetchLicensesFullInfo = async () => {
  const response = await fetch(`${baseUrl}/api/priv/licenses_vital`);
  if (response.ok) {
    const data: FullLicenseInfo[] = await response.json();
    return data;
  } else {
    throw new Error("Could not fetch licenses.");
  }
};

/**
 * Get all licenses that a specific user has access to.
 *
 * @param userId The ID of the user.
 * @returns The user's licenses.
 */
export const fetchLicensesForUser = async (userId: string) => {
  const response = await fetch(
    `${baseUrl}/api/priv/user_licenses/user/${userId}`
  );
  if (response.ok) {
    const data: FullLicenseInfo[] = await response.json();
    return data;
  } else {
    throw new Error("Could not fetch licenses.");
  }
};

/**
 * Get all company licenses for a specific user that they have no access to.
 *
 * @param userId The ID of the user.
 * @returns The company's licenses the user doesn't have.
 */
export const fetchLicensesForUserNoAccess = async (userId: string) => {
  const response = await fetch(
    `${baseUrl}/api/priv/user_licenses/user/${userId}/no_access`
  );

  if (response.ok) {
    const data: FullLicenseInfo[] = await response.json();
    return data;
  } else {
    throw new Error("Could not fetch licenses.");
  }
};

/**
 * Create a license.
 *
 * @param license The license to create.
 */
export const postLicense = async (license: License) => {
  return await fetch(`${baseUrl}/api/priv/licenses`, {
    method: "POST",
    headers: {
      Accept: "application/json",
      "Content-Type": "application/json",
    },
    body: JSON.stringify(license),
    credentials: "include",
  });
};

/**
 * Get all companies.
 *
 * @returns All companies.
 */
export const fetchCompanies = async () => {
  const response = await fetch(`${baseUrl}/api/priv/companies`);
  if (response.ok) {
    const data: Company[] = await response.json();
    return data;
  } else {
    throw new Error("Could not fetch companies.");
  }
};

/**
 * Get all licenses for a company.
 *
 * @param companyId The ID of the company.
 * @returns The licenses for the company.
 */
export const fetchCompanyLicenses = async (companyId: number) => {
  const response = await fetch(
    `${baseUrl}/api/priv/companies/${companyId}/licenses`
  );
  if (response.ok) {
    const data: License[] = await response.json();
    return data;
  } else {
    throw new Error("Could not fetch licenses.");
  }
};

/**
 * Get a specific product.
 *
 * @param productId The ID of the product.
 * @returns The product.
 */
export const fetchProduct = async (productId: string) => {
  const response = await fetch(`${baseUrl}/api/products/${productId}`);
  if (response.ok) {
    const data: Product = await response.json();
    return data;
  } else {
    throw new FetchError(
      "Could not fetch product.",
      response.status,
      response.statusText
    );
  }
};

/**
 * Get all products.
 *
 * @returns All products.
 */
export const fetchProducts = async () => {
  const response = await fetch(`${baseUrl}/api/products`);
  if (response.ok) {
    const data: Product[] = await response.json();
    return data;
  } else {
    throw new Error("Could not fetch products.");
  }
};

/**
 * Get all description components for a specific product.
 *
 * @param productId The ID of the product.
 * @returns The descriptions for the product.
 */
export const fetchDescriptionComponents = async (productId: string) => {
  const response = await fetch(
    `${baseUrl}/api/products/${productId}/descriptions`
  );
  if (response.ok) {
    const data: Description[] = await response.json();
    return data;
  } else {
    throw new Error("Could not fetch descriptions.");
  }
};

/**
 * Get all testimonials for a specific product.
 *
 * @param productId The ID of the product.
 * @returns The testimonials for the product.
 */
export const fetchTestimonials = async (productId: string) => {
  const response = await fetch(`${baseUrl}/api/testimonials/${productId}`);
  if (response.ok) {
    const data: Testimonial[] = await response.json();
    return data;
  } else {
    throw new Error("Could not fetch testimonials.");
  }
};

/**
 * Check if the user is signed in.
 *
 * @returns If the user is signed in.
 */
export const checkSignInStatus = async () => {
  const response = await fetch(`${baseUrl}/api/priv/logged_in`, {
    method: "GET",
    credentials: "include",
  });
  return response.status === 200;
};

/**
 * Verify that the sign in information is valid.
 *
 * @param email The user's email.
 * @param password The user's password.
 * @returns The response from the fetch request.
 */
export const verifySignInInfo = async (email: string, password: string) => {
  return await fetch(`${baseUrl}/api/login`, {
    method: "POST",
    body: JSON.stringify({
      email: email,
      password: password,
    }),
    credentials: "include",
    headers: {
      "Content-Type": "application/json",
      credentials: "include",
    },
  });
};

/**
 * Sign out the user.
 * @returns The response from the fetch request.
 * */
export const logout = async () => {
  return await fetch(`${baseUrl}/api/priv/logout`, {
    method: "POST",
    credentials: "include",
  });
};
