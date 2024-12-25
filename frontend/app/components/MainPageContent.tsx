export default function MainPageContent({songsList}: {songsList: string[]}) {
    return (
    <div id="content">
        <div className="flex flex-col gap-2 p-3">
            {songsList.map((song, index) => {
                return (
                    <div key={index} className="bg-neutral-900 px-4 p-2 rounded-3xl">
                        <span >{song}</span>
                    </div>
                )
            })}
        </div>
    </div>
    )
}