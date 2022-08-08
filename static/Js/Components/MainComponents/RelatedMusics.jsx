import React from 'react';
import PropTypes from 'prop-types';
import { Row, Col } from 'react-bootstrap';
import ButtonIcon from '../Helper/ButtonIcon';
import MusicItemRow from '../Items/MusicItemRow';
import axios from 'axios';

export default class RelatedMusics extends React.Component {
    static propTypes = {
        Musics: PropTypes.array.isRequired,
        OnAdd: PropTypes.func.isRequired,
    };

    constructor(props) {
        super(props);
        this.state = {
            RelatedMusics: [],
            RelatedMusicAdded: false,
        };
    }

    getNewRelatedMusics = () => {
        const { Musics } = this.props;
        const MusicIds = Musics.map((m) => m._id);
        axios.post('/Music/Related', { MusicIds }).then((res) => {
            this.setState({
                RelatedMusics: res.data.RelatedMusics,
            });
        });
    };

    onReloadRelated = () => {
        this.getNewRelatedMusics();
    };

    componentDidMount() {
        this.getNewRelatedMusics();
    }

    componentDidUpdate(prevProps, prevState) {
        const { Musics } = this.props;
        const { RelatedMusicAdded } = this.state;
        if (prevProps.Musics !== Musics) {
            if (!RelatedMusicAdded) {
                this.getNewRelatedMusics();
            } else {
                this.setState({
                    RelatedMusicAdded: false,
                });
            }
        }
    }

    onAdd = (Music) => {
        const { RelatedMusics } = this.state;
        this.props.OnAdd(Music);
        this.setState({
            RelatedMusicAdded: true,
            RelatedMusics: RelatedMusics.filter((m) => m._id !== Music._id),
        });
    };

    render() {
        const { RelatedMusics } = this.state;

        const RelatedMusicsElems = RelatedMusics.map((m) => {
            return (
                <MusicItemRow
                    ImageDz={m.image_url}
                    Title={m.title}
                    Artist={m.artist_name}
                    onClick={() => this.onAdd(m)}
                ></MusicItemRow>
            );
        });

        return (
            <div className="m-4">
                <Row className="p-1">
                    <Col className="mr-auto">
                        <h3 className="align-self-center my-auto">Related</h3>
                    </Col>
                    <Col>
                        <ButtonIcon
                            dataEva={'flip-outline'}
                            onClick={this.onReloadRelated}
                            evaOptions={{
                                fill: '#d6d6d6ff',
                                width: '30px',
                                height: '30px',
                            }}
                            buttonClass="py-auto pr-0 float-right"
                        />
                    </Col>
                </Row>

                <table className="table table-hover table-borderless">
                    <tbody>{RelatedMusicsElems}</tbody>
                </table>
            </div>
        );
    }
}
