import { Link } from "react-router-dom";

interface Props {
  active: boolean;
}

export default function PurchaseLicenseButton(props: Props) {
  if (props.active) {
    return (
      <Link className="hero-button" to="purchase-license">
        Buy license
      </Link>
    );
  } else {
    return (
      <button className="hero-button" disabled>
        Buy license
      </button>
    );
  }
}
