import { Component } from "react";
import LicensePrices from "./LicensePrices";

/**
 * Represents a Purchase License page.
 *
 * Contains a short product description, the plan options, and
 * a checkbox that the user accepts the terms and conditions.
 */
export default class PurchaseLicense extends Component {
  state = {
    productName: "Placeholder Name",
    productDescription:
      "Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatum, beatae cum. Sequi id sint quos beatae! Similique, molestias enim explicabo obcaecati a iste voluptates repellat? Earum a possimus quo itaque.",
    pricePerLicense: 15,
    totalPrice: 0,
  };

  /**
   * Update the total price in the object's state.
   *
   * @param event The user event.
   */
  updatePrice = (event: React.ChangeEvent<HTMLSelectElement>) => {
    this.setState({ totalPrice: event.target.value });
  };

  render() {
    return (
      <section className="container">
        <h1>Purchase License &rarr; {this.state.productName}</h1>
        <form className="left-aligned">
          <p>{this.state.productDescription}</p>

          <LicensePrices
            price={this.state.pricePerLicense}
            updatePrice={this.updatePrice}
          />

          <p className="total-price">TOTAL: {this.state.totalPrice}</p>

          <button type="submit" className="default-button submit-button">
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

  componentDidMount(): void {
    this.validateForm();
  }

  /**
   * Confirm that all the form's input is valid.
   *
   * If the user has not selected an option for the plan,
   * inform the user that their input is needed.
   */
  validateForm(): void {
    const prices: HTMLSelectElement | null = document.querySelector("#prices");
    const submitButton: HTMLInputElement | null =
      document.querySelector(".submit-button");
    const formAlert: HTMLElement | null = document.querySelector(".form-alert");

    submitButton?.addEventListener("click", function (e) {
      if (prices?.selectedIndex == 0) {
        e.preventDefault();
        if (formAlert != null) {
          formAlert.innerHTML = "Please select a plan";
        }
      }
    });
  }
}
