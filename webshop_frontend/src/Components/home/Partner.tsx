import { Link } from "react-router-dom";

export type PartnerProps = {
  partner: {
    name: string;
    link: string;
    imageSize: number[];
    imageSource: string;
  };
};

/**
 * Represents the information about a Partner.
 *
 * @param partner A partner object.
 * @returns The Partner component as a JSX element.
 */
export default function Partner({ partner }: PartnerProps) {
  return (
    <li>
      <Link target="_blank" to={partner.link}>
        <img
          className="partner-logo"
          src={partner.imageSource}
          width={partner.imageSize[0]}
          height={partner.imageSize[1]}
          alt={partner.name + " logo"}
        ></img>
      </Link>
    </li>
  );
}
