import { useState, ChangeEvent, MouseEvent as ReactMouseEvent } from "react";
import LicensePrices from "./LicensePrices";

/**
 * Represents a Purchase License page.
 * Contains a short product description, the plan options, and
 * a checkbox for accepting terms and conditions.
 *
 * @returns The Purchase License page as a JSX element.
 */
export default function PurchaseLicense() {
  const name = "Placeholder Name";
  const description =
    "Lorem ipsum dolor sit amet consectetur adipisicing elit. Delectus, sequi.";
  const pricePerLicense = 15;
  const [totalPrice, setTotalPrice] = useState<number>(0);

  /**
   * Update the total price in the object's state.
   *
   * @param event The user event.
   */
  const updatePrice = (event: ChangeEvent<HTMLSelectElement>) => {
    setTotalPrice(parseInt(event.target.value));
  };

  return (
    <section className="container">
      <h1>Purchase License &rarr; {name}</h1>
      <form className="left-aligned">
        <p>{description}</p>

        <LicensePrices
          price={pricePerLicense}
          updatePrice={(event) => updatePrice(event)}
        />

        <p className="total-price">TOTAL: {totalPrice}</p>

        <button
          type="submit"
          className="default-button submit-button"
          onClick={(event) => validateForm(event)}
        >
          Buy
        </button>
        <div className="checkbox-input">
          <input id="accept-terms" type="checkbox" required />
          <label htmlFor="accept-terms">
            I have read and agree to the <a href="#!">terms of service</a>.
          </label>
        </div>
        <p className="form-alert"></p>
      </form>
    </section>
  );
}

/**
 * Confirm that all the form's input is valid.
 *
 * If the user has not selected an option for the plan,
 * inform the user that their input is needed.
 *
 * @param event Mouse Event on button
 */
function validateForm(event: ReactMouseEvent<HTMLButtonElement, MouseEvent>) {
  const prices: HTMLSelectElement | null = document.querySelector("#prices");
  const formAlert: HTMLElement | null = document.querySelector(".form-alert");

  if (prices?.selectedIndex == 0) {
    event.preventDefault();

    if (formAlert != null) {
      formAlert.innerHTML = "Please select a plan";
    }
  }
}
