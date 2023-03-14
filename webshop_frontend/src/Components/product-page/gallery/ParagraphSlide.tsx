/**
 * The props of the ParagraphSlide component.
 */
export type ParagraphSlideProps = {
  id: string;
  paragraph: string;
  reviewerProfile: {
    picturePath: string;
    name: string;
  };
};

/**
 * Renders a slide that contains a paragraph and a short profile
 * of the reviewer.
 *
 * @param props  the props of the slide, must be of type ParagraphSlideProps
 * @returns the slide HTML element
 */
export const ParagraphSlide = (props: ParagraphSlideProps) => {
  let alt_text = props.reviewerProfile.name + "'s profile picture";
  return (
    <div className="slide" id={props.id}>
      <p>{props.paragraph}</p>
      <div className="reviewer-profile">
        <img
          src={props.reviewerProfile.picturePath}
          alt={props.reviewerProfile.name + "'s face"}
          height="80"
          width="80"
          className="reviewer-picture"
        />
        <div className="reviewer-details">
          <p>{props.reviewerProfile.name}</p>
        </div>
      </div>
    </div>
  );
};
