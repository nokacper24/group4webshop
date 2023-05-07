/**
 * The terms of service with a checkbox.
 *
 * @returns The terms of service checkbox.
 */
export default function TermsOfService() {
  return (
    <div className="checkbox-input">
      <label htmlFor="accept-terms">
        <input id="accept-terms" type="checkbox" required />I have read and
        agree to the terms of service.
      </label>
    </div>
  );
}
