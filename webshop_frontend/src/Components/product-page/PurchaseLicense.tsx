import { ProductCardProps } from "../products/ProductCard";
import LicensePrices from "./LicensePrices";

export default function PurchaseLicense(/* Product */) {
  return (
    <section className="container left-aligned">
      <h1>Purchase License &rarr; Product Name</h1>
      <p>
        {/* Product Description */}
        Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatum,
        beatae cum. Sequi id sint quos beatae! Similique, molestias enim
        explicabo obcaecati a iste voluptates repellat? Earum a possimus quo
        itaque.
      </p>

      <LicensePrices />

      <p className="total-price">TOTAL: </p>
      <button className="default-button">Buy</button>
      <div className="checkbox-input">
        <input id="accept-terms" type="checkbox"></input>
        <label htmlFor="accpept-terms">
          I have read and agree to the <a href="#!">terms of service</a>
        </label>
      </div>
    </section>
  );
}
