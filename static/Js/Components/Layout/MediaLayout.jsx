import React from 'react';
import { Button, Col, Row, Spinner } from 'react-bootstrap';
import PropTypes from 'prop-types';

class MediaLayout extends React.Component {
    static propTypes = {
        title: PropTypes.string.isRequired,
        isLoading: PropTypes.bool,
        accessories: PropTypes.arrayOf(PropTypes.any),
        children: PropTypes.object.isRequired,
        showMore: PropTypes.bool,
        onMoreClick: PropTypes.func,
    };

    static defaultProps = {
        accessories: [],
        showMore: false,
        isLoading: false,
        onMoreClick: () => {},
    };

    render() {
        const { title, isLoading, showMore, onMoreClick, accessories, children } = this.props;

        if (isLoading) {
            return (
                <div className="MusicGroup">
                    <div className="d-flex px-1">
                        <h3 className="align-self-center mr-auto">{title}</h3>
                    </div>
                    <Spinner animation="border" role="status" size="lg">
                        <span className="sr-only">Loading...</span>
                    </Spinner>
                </div>
            );
        }

        return (
            <div className="MusicGroup">
                <div className="d-flex px-1">
                    <h3 className="align-self-center mr-auto">{title}</h3>
                    {accessories.map((accessory) => (
                        <div className="mx-1">{accessory}</div>
                    ))}
                </div>
                {children}
                {showMore && (
                    <div style={{ textAlign: 'center' }}>
                        <Button onClick={onMoreClick} variant="outline-dark">
                            More
                        </Button>
                    </div>
                )}
            </div>
        );
    }
}
export default MediaLayout;
