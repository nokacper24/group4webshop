import { Link } from "react-router-dom";

export type PartnerProps = {
  name: string;
  link: string;
  imageSize: number[];
  imageSource: string;
};

/**
 * Represents the information about a Partner.
 *
 * @param partner A partner object.
 * @returns The Partner component as a JSX element.
 */
export default function Partner(props: PartnerProps) {
  return (
    <li>
      <Link target="_blank" to={props.link}>
        <img
          className="partner-logo"
          src={props.imageSource}
          width={props.imageSize[0]}
          height={props.imageSize[1]}
          alt={props.name + " logo"}
        ></img>
      </Link>
    </li>
  );
}
