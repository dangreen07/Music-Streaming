export default function MainPageContentSkeleton() {
    return (
        <div id="content">
            <div className="flex flex-col gap-2 p-3">
                <div className="bg-neutral-900 px-4 p-2 rounded-3xl animate-pulse">
                    <span className=" h-6 w-1/2 block"></span>
                </div>
                <div className="bg-neutral-900 px-4 p-2 rounded-3xl animate-pulse">
                    <span className=" h-6 w-1/2 block"></span>
                </div>
                <div className="bg-neutral-900 px-4 p-2 rounded-3xl animate-pulse">
                    <span className="h-6 w-1/2 block"></span>
                </div>
            </div>
        </div>
    )
}