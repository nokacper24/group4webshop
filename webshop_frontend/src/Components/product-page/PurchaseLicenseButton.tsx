import { Link } from "react-router-dom";

interface Props {
  active: boolean;
}

export default function PurchaseLicenseButton(props: Props) {
  if (props.active) {
    return (
      <Link to="purchase-license">
        <button className="banner-element hero-button">Buy license</button>
      </Link>
    );
  } else {
    return (
      <button className="banner-element hero-button" disabled>
        Buy license
      </button>
    );
  }
}
