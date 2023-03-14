import { useRef } from "react";
import { Link } from "react-router-dom";

/**
 * Represents the Sign In form for users to access their profile.
 *
 * @returns A Sign In component.
 */
export default function SignIn() {
    let baseUrl = import.meta.env.VITE_URL + ":" + import.meta.env.VITE_PORT+ "/";
// check if we are in production mode
if (import.meta.env.PROD) {
  baseUrl = "../";
}

  const email = useRef<HTMLInputElement>(null);
  const password = useRef<HTMLInputElement>(null);

  const verifySignInInfo = async () => {
    let result = await fetch(baseUrl + "api/login", {
        method: "POST",
        body: JSON.stringify({
            email: email.current?.value,
            password: password.current?.value,
        }),
        headers: {
            "Content-Type": "application/json",
            "credentials": "include",
        },
    })

    if (result.status === 200) {
        console.log("User logged in");
        
    }
    else {
        console.log("User not logged in");
        password.current!.value = "";
    }
  };

  return (
    <section className="center-container">

      <form className="container form-container" onSubmit={(e) => e.preventDefault()}>
        <h1>Sign in</h1>
        <p>
          Not a customer yet?
          <Link to="create-account"> Register here!</Link>
        </p>

        <label htmlFor="sign-in-email">E-mail</label>
        <input
          ref={email}
          id="sign-in-email"
          name="email"
          type="text"
          placeholder="user@company.com"
          required
        ></input>

        <label htmlFor="sign-in-password">Password</label>
        <input
          ref={password}
          id="sign-in-password"
          name="password"
          type="password"
          placeholder="password"
          required
        ></input>

        <button
          onClick={verifySignInInfo}
          className="default-button m-t-1"
          type="submit"
          value="Sign in"
        >
          Sign in
        </button>
      </form>
    </section>
  );
}
