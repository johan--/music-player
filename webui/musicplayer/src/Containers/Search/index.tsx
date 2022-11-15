import { useEffect, useMemo } from "react";
import { useNavigate, useSearchParams } from "react-router-dom";
import SearchResults from "../../Components/SearchResults";
import { useTimeFormat } from "../../Hooks/useFormat";
import { usePlayback } from "../../Hooks/usePlayback";
import { useSearch } from "../../Hooks/useSearch";

const SearchPage = () => {
  const navigate = useNavigate();
  const { formatTime } = useTimeFormat();
  const {
    play,
    pause,
    next,
    previous,
    nowPlaying,
    nextTracks,
    previousTracks,
    playNext,
    playTrackAt,
    removeTrackAt,
  } = usePlayback();
  const [params] = useSearchParams();
  const { onSearch, results, query } = useSearch();
  const q = useMemo(() => params.get("q"), [params]);

  useEffect(() => {
    if (q && q !== null) {
      onSearch(q);
    }
  }, [q]);

  return (
    <>
      <SearchResults
        tracks={results.tracks.map((x) => ({
          ...x,
          time: formatTime(x.duration * 1000),
        }))}
        albums={results.albums}
        artists={results.artists}
        onClickAlbum={({ id }) => navigate(`/albums/${id}`)}
        onClickArtist={({ id }) => navigate(`/artists/${id}`)}
        onClickLibraryItem={(item) => navigate(`/${item}`)}
        onPlay={() => play()}
        onPause={() => pause()}
        onNext={() => next()}
        onPrevious={() => previous()}
        onShuffle={() => {}}
        onRepeat={() => {}}
        nowPlaying={nowPlaying}
        onPlayTrack={(id, position) => {}}
        nextTracks={nextTracks}
        previousTracks={previousTracks}
        onPlayNext={(trackId) => playNext({ variables: { trackId } })}
        onPlayTrackAt={(position) => playTrackAt({ variables: { position } })}
        onRemoveTrackAt={(position) =>
          removeTrackAt({ variables: { position } })
        }
        onSearch={onSearch}
      />
    </>
  );
};

export default SearchPage;