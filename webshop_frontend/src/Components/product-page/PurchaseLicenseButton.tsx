import { Link } from "react-router-dom";

export default function PurchaseLicenseButton() {
  return (
    <Link to="purchase-license">
      <button className="banner-element hero-button">Buy license</button>
    </Link>
  );
}
