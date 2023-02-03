import React, { Component } from "react";

export default class LicensePrices extends Component {
  state = {
    priceTiers: [
      1, 2, 3, 4, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 60, 70, 80, 90, 100,
      150, 200, 250, 300,
    ],
    price: 10,
  };

  render() {
    return (
      <React.Fragment>
        <label htmlFor="prices">Choose a plan:</label>
        <select id="prices" name="prices">
          {this.state.priceTiers.map((tier: number) => (
            <option key={tier} value={tier}>
              {tier + " user(s) — $" + tier * this.state.price + "/year"}
            </option>
          ))}
        </select>

        {/* <label htmlFor="prices">Choose a plan:</label>
        <select id="prices" name="prices">
          <option value="volvo">Volvo</option>
          <option value="saab">Saab</option>
          <option value="fiat">Fiat</option>
          <option value="audi">Audi</option>
        </select> */}
      </React.Fragment>
    );
  }
}
