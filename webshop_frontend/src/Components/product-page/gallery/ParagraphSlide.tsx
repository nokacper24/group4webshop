/**
 * The props of the ParagraphSlide component.
 */
export type ParagraphSlideProps = {
  paragraph: string;
  imagePath: string;
  name: string;
};

/**
 * Renders a slide that contains a paragraph and a short profile
 * of the reviewer.
 *
 * @param props  the props of the slide, must be of type ParagraphSlideProps
 * @returns the slide HTML element
 */
export const ParagraphSlide = (props: ParagraphSlideProps) => {
  return (
    <div className="slide">
      <p>{props.paragraph}</p>
      <div className="reviewer-profile">
        <img
          src={props.imagePath}
          alt={props.name}
          height="80"
          width="80"
          className="reviewer-picture"
        />
        <div className="reviewer-details">
          <p>{props.name}</p>
        </div>
      </div>
    </div>
  );
};
