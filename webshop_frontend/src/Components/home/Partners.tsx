import { Partner } from "./Partner";

type PartnersProps = {
  partners: {
    name: string;
    link: string;
    imageSize: number[];
    imageSource: string;
  }[];
};

/**
 * Represents a list of Partner components.
 *
 * @param partners A list of Partner objects.
 * @returns The Partners component as a JSX element.
 */
export const Partners = ({ partners }: PartnersProps) => {
  return (
    <ul className="partners">
      {partners.map((partner) => (
        <Partner key={partner.name} partner={partner} />
      ))}
    </ul>
  );
};
