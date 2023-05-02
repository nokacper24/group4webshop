/**
 * The terms of service with a checkbox.
 *
 * @returns The terms of service checkbox.
 */
export default function TermsOfService() {
  return (
    <div className="checkbox-input">
      <input id="accept-terms" type="checkbox" required />
      <label htmlFor="accept-terms">
        I have read and agree to the terms of service.
      </label>
    </div>
  );
}
