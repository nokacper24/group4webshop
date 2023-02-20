import React, { useEffect, useState } from "react";
import { Link } from "react-router-dom";
import DescriptionRow, { ProductPageRow } from "./DescriptionRow";
import Gallery from "./gallery/Gallery";
import { GalleryProps } from "./gallery/Gallery";
import { SlideType } from "./gallery/SlideTypes";

let counter = 0;

export type State = {
  rows: ProductPageRow[];
};

export interface Product {
    product_id: string;
    display_name: string;
    price_per_user: number;
    short_description: string;
    main_image: string;
    available: boolean;
}


export interface Text {
    text_title: string;
    paragraph: string;
}

export interface Image {
    image_path: string;
    alt_text: string;
}

export interface Description {
    component_id: number;
    priority: number;
    product_id: string;
    text: Text;
    image: Image;
    isTextNotImage: boolean;
}


let baseUrl = "http://localhost:8081";


/**
 * The product page component.
 *
 * @returns the product page component
 */
export default function ProductPage() {

    const [product, setProduct] = useState<Product>();
    // array of rows
    const [descriptionRow, setDescriptionRow] = useState<ProductPageRow[]>([]);

    //get product id from url
    const productId = window.location.pathname.split("/")[2];

    const fetchProduct = async () => {
        const response = await fetch(`${baseUrl}/api/products/${productId}`);
        const data = await response.json();
        const product = {
            product_id: data.product_id,
            display_name: data.display_name,
            price_per_user: data.price_per_user,
            short_description: data.short_description,
            main_image: data.main_image,
            available: data.available,
        };
        setProduct(product);
    };

    const fetchDescriptions = async () => {
        const response = await fetch(`${baseUrl}/api/products/${productId}/description`);
        const data = await response.json();
        const descriptions = data.map((description: any) => {
            // check if text or image, by seeing if either text or image is null

            if (description.text == null) {
                return {
                    component_id: description.component_id,
                    priority: description.priority,
                    product_id: description.product_id,
                    text: {
                        text_title: null,
                        paragraph: null,
                    },
                    image: {
                        image_path: description.image.image_path,
                        alt_text: description.image.alt_text,
                    },
                    isTextNotImage: true,
                };
            } else {
                return {
                    component_id: description.component_id,
                    priority: description.priority,
                    product_id: description.product_id,
                    text: {
                        text_title: description.text.text_title,
                        paragraph: description.text.paragraph,
                    },
                    image: {
                        image_path: null,
                        alt_text: null,
                    },
                    isTextNotImage: false
                };
            }
            
        });

        // sort by priority
        descriptions.sort((a: { priority: number; }, b: { priority: number; }) => a.priority - b.priority);
        
        // create rows
        let rows: ProductPageRow[] = [];
        
        descriptions.forEach((description: Description) => {
            // check if description is text or image
            if (description.isTextNotImage) {
                // create row with text
                let row: ProductPageRow = {
                    props: {
                        item1: {
                            title: description.text.text_title,
                            content: description.text.paragraph,
                            isTextNotImage: true,
                    },
                    item2: {
                        title: "",
                        content: description.image.image_path,
                        isTextNotImage: false,
                    },
                    textToLeft: true,
                }
                }
                rows.push(row);
            }
                else {
                    // create row with image
                    let row: ProductPageRow = {
                        props: {
                            item1: {
                                title: "",
                                content: description.image.image_path,
                                isTextNotImage: false,
                        },
                        item2: {
                            title: description.text.text_title,
                            content: description.text.paragraph,
                            isTextNotImage: true,
                        },
                        textToLeft: false,

                }
                
            }
            rows.push(row);
        }
    });

    setDescriptionRow(rows);
    };

       
    

    useEffect(() => {
        fetchProduct();
        fetchDescriptions();
    }, []);
   



  
  let testimonial: GalleryProps = {
    galleryName: "testGallery",
    slides: [
      {
        slideId: "slide1",
        mainContent:
          "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nam sapien orci, varius quis mauris a, blandit imperdiet tellus. Donec a cursus leo. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras tincidunt ex vel libero porttitor, quis vulputate mauris condimentum. Mauris blandit purus at mauris fringilla pretium. Donec pharetra justo in ultricies accumsan. Duis ullamcorper condimentum porttitor. Nunc pellentesque vestibulum est, et dictum metus pellentesque nec. Morbi luctus turpis vitae facilisis tristique. Duis sed posuere magna. Aliquam sodales, turpis in consequat tristique, nibh odio luctus libero, quis fringilla metus turpis vitae lorem.",
        reviewerProfile: {
          picturePath: "https://picsum.photos/100",
          name: "Joe Kerr",
          title: "Professional Clown",
        },
        slideType: SlideType.PARAGRAPH,
      },
      {
        slideId: "slide2",
        mainContent:
          "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nunc in tellus nibh. Interdum et malesuada fames ac ante ipsum primis in faucibus. Quisque a malesuada nunc, quis aliquam ante. Nulla elementum rutrum magna quis condimentum. Integer nunc enim, semper ut sodales eu, blandit quis leo. Ut blandit mollis est, sit amet ultrices ligula facilisis sed. Morbi non est rutrum, aliquet metus non, iaculis nisl. Donec nec magna hendrerit, elementum elit sit amet, sodales tortor. Pellentesque nulla orci, tincidunt vel lacinia condimentum, euismod ut mi. Integer tristique metus a eros luctus, ac sollicitudin dui iaculis. Vestibulum iaculis consequat dui a lacinia. Fusce id leo eu eros fringilla efficitur id vel nisi.",
        reviewerProfile: {
          picturePath: "https://picsum.photos/100",
          name: "Bat mann",
          title: "Genius acrobat",
        },
        slideType: SlideType.PARAGRAPH,
      },
      {
        slideId: "slide3",
        mainContent:
          "Phasellus id nibh eget justo blandit rhoncus et ut libero. Nunc ullamcorper, elit id interdum faucibus, leo ipsum tristique libero, nec varius ante nunc ut purus. In blandit in odio vel convallis. Curabitur non elementum elit, sed vestibulum dui. Phasellus eu dolor magna. Maecenas viverra orci id pellentesque auctor. Duis eu efficitur nunc. Proin ut interdum est. Proin sed volutpat tellus, venenatis dictum augue. Cras ante enim, convallis quis enim eget, scelerisque aliquam nibh. Nunc id sagittis dolor. Praesent luctus et felis vitae laoreet. Quisque ultricies sapien risus, in faucibus odio faucibus non. Aliquam erat volutpat. Proin consectetur blandit ex in aliquam.",
        reviewerProfile: {
          picturePath: "https://picsum.photos/100",
          name: "Pene Guin",
          title: "Animal",
        },
        slideType: SlideType.PARAGRAPH,
      },
    ],
  };
  return (
    
    <React.Fragment>
        {product && (
        <>
      <section className="banner">
        <div className="banner-inner">
          <div className="banner-highlight">
            <h1 className="banner-title banner-element hero-title">
                {product.display_name}
            </h1>
            <Link to="purchase-license">
              <button className="banner-element hero-button">
                Buy license
              </button>
            </Link>
          </div>
        </div>
      </section>
      <hr></hr>
      <section className="product-description container">
        {descriptionRow.map((productPageRow) => (
          <DescriptionRow
            key={assignUniqueKey("row")}
            props={productPageRow.props}
          />
        ))}
      </section>
      {testimonial.slides.length > 0 && (
        <section className="gallery-wrapper">
          <div className="container">
            <h2 className="testimonial-title">Testimonials</h2>
            <Gallery
              slides={testimonial.slides}
              galleryName={testimonial.galleryName}
            />
          </div>
        </section>
      )}
      <section className="container">
        <h2>Purchase</h2>
        <p>Purchase licenses for this product for your enterprise today!</p>
        <Link to="purchase-license">
          <button className="banner-element hero-button">Buy license</button>
        </Link>
      </section>
        </>
        )}
        {!product && (
            <div className="container">
                <h1>Product not found</h1>
                </div>
                )
        }
    </React.Fragment>
  );
}
/**
 * When you have multiple items with the same key, this adds an unique ID behind the key
 * and returns it.
 *
 * @param base the string you want an unique ID added to
 * @returns the string modified to include the ID
 */
function assignUniqueKey(base: string) {
  counter++;
  return base + counter;
}
