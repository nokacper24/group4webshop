import { Link } from "react-router-dom";

/**
 * Represents the footer.
 *
 * @returns Footer component as a JSX element.
 */
export default function Footer() {
  return (
    <footer>
      <p>
        <b>Disclaimer:</b> This website is a result of a university group
        project, performed in the course IDATA2301 Web technologies, at NTNU.
        All the information provided here is a result of imagination. Any
        resemblance with real companies or products is a coincidence. Credits to
        all images used can be found under <Link to="/credits">credits</Link>{" "}
        page.
      </p>
    </footer>
  );
}
