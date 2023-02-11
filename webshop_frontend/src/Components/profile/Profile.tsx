import { useState, useEffect } from "react";
import MyAccount from "./MyAccount";
import SignIn from "./SignIn";

/**
 * The user Profile page.
 * If the user is already signed in, show the My Account page.
 * If the user is not signed in, show the Sign In page.
 *
 * @returns The Profile page component.
 */
export default function Profile() {
  const [authenticated, setAuthenticated] = useState<boolean>(false);

  useEffect(() => {
    if (authenticatedUser()) {
      setAuthenticated(true);
    }
  }, []);

  if (!authenticated) {
    return <SignIn />;
  } else {
    return <MyAccount />;
  }
}

/**
 * Check if the user is authenticated.
 *
 * @returns true the user is authenticated, false if not.
 */
function authenticatedUser(): boolean {
  // Authenticate token
  // Placeholder authentication
  return localStorage.getItem("authenticated") != null;
}
