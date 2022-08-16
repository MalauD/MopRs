import React from 'react';
import PropTypes from 'prop-types';
import { Row, Col } from 'react-bootstrap';
import ButtonIcon from '../Helper/ButtonIcon';
import axios from 'axios';
import MusicElement from '../Elements/MusicElement';
import MusicGroup from './Groups/MusicGroup';

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
            isLoading: false,
        };
    }

    getNewRelatedMusics = () => {
        const { Musics } = this.props;
        const MusicIds = Musics.map((m) => m._id);
        this.setState({
            isLoading: true,
        });
        axios.post('/Music/Related', { MusicIds }).then((res) => {
            this.setState({
                RelatedMusics: res.data.RelatedMusics,
                isLoading: false,
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
        const { isLoading } = this.state;

        const Accessories = [
            <ButtonIcon
                dataEva={'flip-outline'}
                onClick={this.onReloadRelated}
                evaOptions={{
                    fill: '#d6d6d6ff',
                    width: '30px',
                    height: '30px',
                }}
            />,
        ];

        return (
            <MusicGroup
                Musics={RelatedMusics}
                title="Related"
                isLoading={isLoading}
                Accessories={Accessories}
            />
        );
    }
}
