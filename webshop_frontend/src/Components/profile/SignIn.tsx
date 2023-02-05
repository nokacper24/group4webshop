import { Component } from "react";
import { Link } from "react-router-dom";

/**
 * Represents the Sign In form for users to access their profile.
 */
export default class SignIn extends Component {
  render() {
    return (
      <div className="center-container">
        <form className="container form-container">
          <h1>Sign in</h1>
          <p>
            Not a customer yet?
            <Link to="/create-account"> Register here!</Link>
          </p>

          <label htmlFor="sign-in-email">E-mail</label>
          <input
            id="sign-in-email"
            name="email"
            type="text"
            placeholder="user@company.com"
            required
          ></input>

          <label htmlFor="sign-in-password">Password</label>
          <input
            id="sign-in-password"
            name="password"
            type="password"
            placeholder="password"
            required
          ></input>

          <button
            className="default-button m-t-1"
            type="submit"
            value="Sign in"
          >
            Sign in
          </button>
        </form>
      </div>
    );
  }
}
