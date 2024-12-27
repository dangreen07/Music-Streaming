export default function MainPageContentSkeleton() {
    return (
        <div id="content">
            <div className="flex flex-col gap-2 p-3">
                {Array.from(Array(5).keys()).map((_current, index) => {
                    return (
                        <div key={index} className="bg-neutral-900 rounded-md animate-pulse">
                            <span className="h-16 w-1/2 block"></span>
                        </div>
                    )
                })}
            </div>
        </div>
    )
}