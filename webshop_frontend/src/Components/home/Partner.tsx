import { Link } from "react-router-dom";

export type PartnerProps = {
  partner: {
    name: string;
    link: string;
    imageSize: number[];
    imageSource: string;
  };
};

export const Partner = ({ partner }: PartnerProps) => {
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
};
