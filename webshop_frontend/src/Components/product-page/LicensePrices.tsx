import React from "react";

export type LicensePricesProps = {
  price: number;
  updatePrice: (arg: React.ChangeEvent<HTMLSelectElement>) => void;
};

/**
 * Represents a select element with the license prices as options.
 *
 * @returns A License Prices component.
 */
export default function LicensePrices(props: LicensePricesProps) {
  const prices = {
    priceTiers: [
      1, 2, 3, 4, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 60, 70, 80, 90, 100,
      150, 200, 250, 300,
    ],
    price: props.price,
  };

  return (
    <React.Fragment>
      <label htmlFor="prices">Choose a plan: </label>
      <select
        id="prices"
        name="prices"
        onChange={props.updatePrice}
        defaultValue="0"
      >
        <option key="0" value="0" disabled hidden>
          Please choose a plan
        </option>
        {prices.priceTiers.map((tier: number) => (
          <option
            key={tier}
            value={Math.round(tier * prices.price * 100) / 100}
          >
            {tier +
              " user(s) — $" +
              Math.round(tier * prices.price * 100) / 100 +
              " / year"}
          </option>
        ))}
      </select>
    </React.Fragment>
  );
}
