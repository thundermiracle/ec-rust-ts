import { FC } from 'react';
import { Skeleton } from '@/components/ui/skeleton';

interface SidebarSkeletonProps {
  count?: number;
  showColorCircle?: boolean;
}

const SidebarSkeleton: FC<SidebarSkeletonProps> = ({ count = 3, showColorCircle = false }) => {
  return (
    <div className="space-y-1 pt-4">
      {Array.from({ length: count }).map((_, index) => (
        <div key={index} className="px-2 py-2 h-auto">
          {showColorCircle ? (
            <div className="flex items-center gap-3">
              <Skeleton className="w-3 h-3 rounded-full flex-shrink-0" />
              <Skeleton className="h-4 w-20" />
            </div>
          ) : (
            <Skeleton className="h-4 w-24" />
          )}
        </div>
      ))}
    </div>
  );
};

export { SidebarSkeleton }; 