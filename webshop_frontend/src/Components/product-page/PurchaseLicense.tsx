import { Component } from "react";
import LicensePrices from "./LicensePrices";

export default class PurchaseLicense extends Component {
  state = {
    productName: "Placeholder Name",
    productDescription:
      "Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatum, beatae cum. Sequi id sint quos beatae! Similique, molestias enim explicabo obcaecati a iste voluptates repellat? Earum a possimus quo itaque.",
    pricePerLicense: 15,
    totalPrice: 0,
  };

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

          <button type="submit" className="default-button">
            Buy
          </button>
          <div className="checkbox-input">
            <input id="accept-terms" type="checkbox" required />
            <label htmlFor="accept-terms">
              I have read and agree to the <a href="#!">terms of service</a>.
            </label>
          </div>
        </form>
      </section>
    );
  }
}
