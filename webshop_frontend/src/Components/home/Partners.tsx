import Partner from "./Partner";

/**
 * The list of ProFlex' partners.
 */
const partners = [
  {
    name: "Acme",
    link: "https://logoipsum.com/",
    imageSize: [200, 100],
    imageSource: "https://img.logoipsum.com/289.svg",
  },
  {
    name: "Acme2",
    link: "https://logoipsum.com/",
    imageSize: [200, 100],
    imageSource: "https://img.logoipsum.com/235.svg",
  },
  {
    name: "Acme3",
    link: "https://logoipsum.com/",
    imageSize: [200, 100],
    imageSource: "https://img.logoipsum.com/260.svg",
  },
  {
    name: "Acme4",
    link: "https://logoipsum.com/",
    imageSize: [200, 100],
    imageSource: "https://img.logoipsum.com/254.svg",
  },
  {
    name: "Acme5",
    link: "https://logoipsum.com/",
    imageSize: [200, 100],
    imageSource: "https://img.logoipsum.com/250.svg",
  },
  {
    name: "Acme6",
    link: "https://logoipsum.com/",
    imageSize: [200, 100],
    imageSource: "https://img.logoipsum.com/251.svg",
  },
  {
    name: "Acme7",
    link: "https://logoipsum.com/",
    imageSize: [200, 100],
    imageSource: "https://img.logoipsum.com/253.svg",
  },
];

/**
 * Represents a list of Partner components.
 *
 * @returns The Partners component as a JSX element.
 */
export default function Partners() {
  return (
    <ul className="partners">
      {partners.map((partner) => (
        <Partner
          key={partner.name}
          name={partner.name}
          link={partner.link}
          imageSize={partner.imageSize}
          imageSource={partner.imageSource}
        />
      ))}
    </ul>
  );
}
