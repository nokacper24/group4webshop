import { GalleryProps } from "./Gallery";

/**
 * The props of the ParagraphSlide component.
 */
export type ParagraphSlideProps = {
  id: string;
  paragraph: string;
  reviewerProfile: {
    picturePath: string;
    name: string;
    title: string;
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
  return (
    <div className="slide" id={props.id}>
      <p>{props.paragraph}</p>
      <div className="reviewer-profile">
        <img
          src={props.reviewerProfile.picturePath}
          alt=""
          className="reviewer-picture"
        />
        <div className="reviewer-details">
          <p>{props.reviewerProfile.name}</p>
          <p>{props.reviewerProfile.title}</p>
        </div>
      </div>
    </div>
  );
};
