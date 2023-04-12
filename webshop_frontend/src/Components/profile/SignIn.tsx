import { useRef } from "react";
import { Link, useNavigate } from "react-router-dom";
import { verifySignInInfo } from "../../ApiController";

/**
 * Represents the Sign In form for users to access their profile.
 *
 * @returns A Sign In component.
 */
export default function SignIn() {
  const navigate = useNavigate();

  const email = useRef<HTMLInputElement>(null);
  const password = useRef<HTMLInputElement>(null);

  const handleSignIn = () => {
    verifySignInInfo(email.current!.value, password.current!.value).then(
      (response: Response) => {
        if (response.status === 200) {
          // Refresh
          navigate(0);
        } else {
          password.current!.value = "";
        }
      }
    );
  };

  return (
    <section className="center-container">
      <form
        className="container form-container"
        onSubmit={(e) => e.preventDefault()}
      >
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
          onClick={handleSignIn}
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
