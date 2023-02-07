import { SlideType } from "./SlideTypes";

export type SlideProps = {
  mainContent: string;
  reviewerProfile: {
    picturePath: string;
    name: string;
    title: string;
  };
  slideType: SlideType;
};

export default function (slideProps: SlideProps[]) {
  let slide: JSX.Element[];

  slideProps.forEach(function (prop) {
    switch (prop.slideType) {
      case SlideType.PARAGRAPH: {
      }
    }
  });

  return <div></div>;
}
