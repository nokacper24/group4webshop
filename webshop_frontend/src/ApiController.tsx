import {
  Company,
  Description,
  License,
  LicenseVital,
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
 * Get a specific user.
 *
 * @param userId The ID of the user.
 * @returns The user.
 */
export const fetchUser = async (userId: string) => {
  const response = await fetch(`${baseUrl}/api/priv/users/${userId}`);
  const data: User = await response.json();
  return data;
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
  const data: MeUser = await response.json();
  return data;
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
  const data: User[] = await response.json();
  return data;
};

/**
 * Get a specific license.
 *
 * @param licenseId The ID of the license.
 * @returns The license.
 */
export const fetchLicense = async (licenseId: string) => {
  const response = await fetch(`${baseUrl}/api/licenses/${licenseId}`);
  const data: License = await response.json();
  return data;
};

/**
 * Get all licenses' vital information.
 *
 * @returns All licenses' vital information.
 */
export const fetchLicensesVital = async () => {
  const response = await fetch(`${baseUrl}/api/licenses_vital`);
  const data: LicenseVital[] = await response.json();
  return data;
};

/**
 * Get all licenses that a specific user has access to.
 *
 * @param userId The ID of the user.
 * @returns The user's licenses.
 */
export const fetchLicensesForUser = async (userId: string) => {
  const response = await fetch(`${baseUrl}/api/user_licenses/user/${userId}`);
  const data: License[] = await response.json();
  return data;
};

/**
 * Get all company licenses for a specific user that they have no access to.
 *
 * @param userId The ID of the user.
 * @returns The company's licenses the user doesn't have.
 */
export const fetchLicensesForUserNoAccess = async (userId: string) => {
  const response = await fetch(
    `${baseUrl}/api/user_licenses/user/${userId}/no_access`
  );
  const data: License[] = await response.json();
  return data;
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
  });
};

/**
 * Get all companies.
 *
 * @returns All companies.
 */
export const fetchCompanies = async () => {
  const response = await fetch(`${baseUrl}/api/companies`);
  const data: Company[] = await response.json();
  return data;
};

/**
 * Get all licenses for a company.
 *
 * @param companyId The ID of the company.
 * @returns The licenses for the company.
 */
export const fetchCompanyLicenses = async (companyId: number) => {
  const response = await fetch(
    `${baseUrl}/api/companies/${companyId}/licenses`
  );
  const data: License[] = await response.json();
  return data;
};

/**
 * Get a specific product.
 *
 * @param productId The ID of the product.
 * @returns The product.
 */
export const fetchProduct = async (productId: string) => {
  const response = await fetch(`${baseUrl}/api/products/${productId}`);
  const data: Product = await response.json();
  return data;
};

/**
 * Get all products.
 *
 * @returns All products.
 */
export const fetchProducts = async () => {
  const response = await fetch(`${baseUrl}/api/products`);
  const data: Product[] = await response.json();
  return data;
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
  const data: Description[] = await response.json();
  return data;
};

/**
 * Get all testimonials for a specific product.
 *
 * @param productId The ID of the product.
 * @returns The testimonials for the product.
 */
export const fetchTestimonials = async (productId: string) => {
  const response = await fetch(`${baseUrl}/api/testimonials/${productId}`);
  const data: Testimonial[] = await response.json();
  return data;
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
