/**
 * Represents the Verify component on the Create Account page.
 *
 * @returns The Create Account verification component as a JSX element.
 */
export function EmailVerify() {
  return (
    <>
      <p>
        We sent you an e-mail with a verification link. Please follow the
        instructions in the e-mail to verify your account. It may take a few
        minutes to appear in your inbox.
      </p>
      <p>
        Do not see the e-mail? Check your spam folder.<br></br>
        If you did not receive it, <a href="#">resend e-mail</a>.
      </p>
    </>
  );
}
