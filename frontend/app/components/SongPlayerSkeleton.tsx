export default function SongPlayerSkeleton() {
    return (
        <div className="absolute bottom-0 flex items-center justify-center w-full bg-neutral-900 h-[6.5rem] gap-2">
            <span className="loading loading-ring loading-lg"></span>
        </div>
    );
}