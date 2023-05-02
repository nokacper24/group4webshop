import { useRef } from "react";
import { useNavigate } from "react-router-dom";

/**
 * Represents the Register E-mail component on the Create Account page.
 *
 * Allows the user to register their e-mail, and receive a
 * verification link to create their account and company.
 *
 * @returns The Register E-mail component.
 */
export function RegisterEmail() {
  const navigate = useNavigate();

  const email = useRef<HTMLInputElement>(null);

  const handleSubmit = (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    // TODO: Fix
    console.log(email.current!.value);
    navigate("/register/verify");
  };

  return (
    <>
      <p>
        Type in your business e-mail and we will send you a registration link.
        This is only intended for managers to register their company.
      </p>
      <p>
        Is your company already registered, or you're not a license manager?
        Please contact yours to get a direct invitation link.
      </p>

      <form onSubmit={(event) => handleSubmit(event)}>
        <label htmlFor="register-email">E-mail</label>
        <input
          ref={email}
          id="register-email"
          name="email"
          type="text"
          placeholder="user@company.com"
          required
        ></input>

        <button className="default-button m-t-1" type="submit">
          Send verification
        </button>
      </form>
    </>
  );
}
